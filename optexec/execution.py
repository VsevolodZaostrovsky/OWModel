import numpy as np
from numba import njit, int64, prange
from math import sqrt, sinh, cosh, acosh, log, exp

import pytest

from . import dataloader


@njit
def sigma_2_rho(ret):
    return sqrt(np.var(ret))


class TWAP:
    """
        Time-Weighted Average Price strategy
    """

    def __init__(self, T: int, L: int, W: np.ndarray) -> None:

        if T <= 0 or L <= 0:
            raise (ValueError)
        if L > T:
            raise (ValueError)

        self.T = T
        self.L = L
        self.W = W
        self.num_of_rounds = W.shape[0]

        # idx = np.random.randint(low = 0, high = N, size = X - (X//N)*N)
        self.trading_list = (
            np.ones(shape=(self.num_of_rounds, L), dtype=int).T * (W // L)).T
        self.trading_list[:, -1] += (W - (W // L) * L)

    def cumulative_impact(self,
                          orderbook_bid: dataloader.OnlineData,
                          orderbook_ask: dataloader.OnlineData,
                          skip_rounds: int = 0):
        """ 
            Average Cost Per Round metric
        """
        for _ in range(skip_rounds):
            for l in range(self.T):
                next(orderbook_bid)
                next(orderbook_ask)

        ACPR = 0.
        dt = np.ediff1d(np.linspace(start=0,
                                    stop=self.T,
                                    num=self.L,
                                    endpoint=True,
                                    dtype=int),
                        to_begin=0)
        for rho in range(self.num_of_rounds):
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            if F1 is None or F2 is None:
                continue

            prW = (F2(1) + F1(1)) / 2 * self.W[rho]
            S = 0
            MI = 0
            for l in range(0, self.L):
                for _ in range(dt[l]):
                    next(orderbook_bid)
                    next(orderbook_ask)
                MI = iter(orderbook_bid).F(self.trading_list[rho, l])
                S += MI
            ACPR += (1 - S / (prW))
        return ACPR / self.num_of_rounds

    def reset(self):
        self.__init__(self.T, self.L, self.W)


##############################################################################
# Time-weighted Average Price Unit Tests
##############################################################################


def test_TWAP_init1():
    strategy1 = TWAP(2000, 100, np.array([100]))
    print(
        "Testing for the correct TWAP intialisation with T = 2000, L = 100, W = 100"
    )
    assert np.sum(strategy1.trading_list) == strategy1.W[0]


def test_TWAP_init2():
    strategy2 = TWAP(2000, 1003, np.array([100]))
    print(
        "Testing for the correct TWAP intialisation with T = 2000, L = 1003, W = 100"
    )
    assert np.sum(strategy2.trading_list) == strategy2.W[0]


def test_TWAP_init3():
    strategy3 = TWAP(2000, 100, np.array([1003]))
    print(
        "Testing for the correct TWAP intialisation with T = 2000, L = 100, W = 1003"
    )
    assert np.sum(strategy3.trading_list) == strategy3.W[0]


##############################################################################
##############################################################################


@njit
def A_l_star(lamb, eta, sigma, W_rho, L, l, gamma=0.):
    tilde_kappa_2 = lamb * sigma / (eta - 0.5 * gamma)
    kappa = acosh(0.5 * tilde_kappa_2 + 1.)
    A = 2. * sinh(kappa / 2.) * cosh(kappa *
                                     (L - l + 0.5)) * W_rho / sinh(kappa * L)
    return A


@njit
def compute_Al(lamb, eta, sigma, W, L, gamma=0):
    W_remain = W
    A = np.empty(L, dtype=int64)
    for l in range(L - 1):
        A[l] = int(A_l_star(lamb, eta, sigma, W, L, l + 1, gamma))
        W_remain -= A[l]
    A[L - 1] = W_remain
    return A


class AC:

    def __init__(self,
                 T: int,
                 L: int,
                 W: np.ndarray,
                 lamb: float,
                 eta: float,
                 init_sigma: float,
                 gamma: float = 0):
        if T <= 0 or L <= 0:
            raise (ValueError)
        if L > T:
            raise (ValueError)

        self.T = T
        self.L = L
        self.W = W
        self.W_remain = np.copy(W)
        self.num_of_rounds = W.shape[0]
        self.tau = (1. * T) / L
        self.gamma = gamma
        self.eta = eta
        self.lamb = lamb
        self.init_sigma = init_sigma

    def cumulative_impact(self,
                          orderbook_bid: dataloader.OnlineData,
                          orderbook_ask: dataloader.OnlineData,
                          rounds_for_est: int = 0):
        """ 
        Average Cost Per Round metric
        """
        ACPR = 0.
        ret = np.empty(shape=self.num_of_rounds + rounds_for_est)
        dt = np.ediff1d(np.linspace(start=0,
                                    stop=self.T,
                                    num=self.L,
                                    endpoint=True,
                                    dtype=int),
                        to_begin=0)
        sigma = self.init_sigma

        for rho in range(rounds_for_est):
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            pr1 = (F2(1) + F1(1)) / 2
            for l in range(self.T):
                next(orderbook_bid)
                next(orderbook_ask)
            F1 = iter(orderbook_bid).F
            F2 = iter(orderbook_ask).F
            prL = (F2(1) + F1(1)) / 2
            ret[rho] = log(prL / pr1)

        for rho in range(self.num_of_rounds):
            sigma = sigma_2_rho(ret[:rho + rounds_for_est]) + 1e-8
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            pr1 = (F2(1) + F1(1)) / 2
            prW = pr1 * self.W[rho]
            S = 0
            A = compute_Al(self.lamb, self.eta, sigma, self.W[rho], self.L,
                           self.gamma)

            for l in range(0, self.L):

                for _ in range(dt[l]):
                    next(orderbook_bid)
                    next(orderbook_ask)

                S += iter(orderbook_bid).F(A[l])
                self.W_remain[rho] -= A[l]

            F1 = iter(orderbook_bid).F
            F2 = iter(orderbook_ask).F
            self.W_remain[rho] = 0
            ACPR += (1 - S / (prW))
            prL = (F2(1) + F1(1)) / 2
            ret[rho + rounds_for_est] = log(prL / pr1)
        return ACPR / self.num_of_rounds

    def reset(self):
        self.__init__(self.T, self.L, self.W, self.lamb, self.eta,
                      self.init_sigma, self.gamma)


@njit
def Cx(state, a, W, pr, sigma, B):
    return a * (B / 2 - state * sigma) / (W * pr)


@njit(parallel=True)
def optimal_policy(A, P, M, W_max, pr, B, sigma, W, Policies):

    L = A.shape[0]
    V = np.empty(shape=(W_max + 1, len(M)))

    for j, m in enumerate(M):
        for i in range(W_max + 1):
            Policies[L - 1, i, j] = i
            V[i, j] = Cx(m, i, W, pr, sigma, B)

    for l in range(L - 2, -1, -1):
        for j in prange(len(M)):
            CX_a = Cx(M[j], A[l], W, pr, sigma, B)
            CX_0 = 0
            for i in range(W_max + 1):
                S1 = CX_a
                S2 = CX_0
                if i - A[l] <= 0:
                    Policies[l, i, j] = 0
                    for m_prime, p in enumerate(P[j]):
                        S2 += p * V[i, m_prime]
                    V[i, j] = S2
                else:
                    for m_prime, p in enumerate(P[j]):
                        S1 += p * V[i - A[l], m_prime]
                        S2 += p * V[i, m_prime]
                    if S1 > S2:
                        V[i, j] = S2
                        Policies[l, i, j] = 0
                    else:
                        V[i, j] = S1
                        Policies[l, i, j] = A[l]


@njit(parallel=True)
def optimal_policy_plus(A, P, M, W_max, pr, B, sigma, W, Policies):

    L = A.shape[0]
    V = np.empty(shape=(W_max + 1, len(M)))
    a_max = np.max(A)
    S = np.empty(shape=(len(M), 2 * a_max))
    CX = np.empty_like(S)

    for j, m in enumerate(M):
        for i in range(W_max + 1):
            Policies[L - 1, i, j] = i
            V[i, j] = Cx(m, i, W, pr, sigma, B)

    for l in range(L - 2, -1, -1):
        for j in prange(len(M)):
            CX[j, 0] = 0
            for a in range(1, 2 * A[l]):
                CX[j, a] = Cx(M[j], a, W, pr, sigma, B)

            for i in range(W_max + 1):
                S[j, 0] = CX[j, 0]
                for m_prime, p in enumerate(P[j]):
                    S[j, 0] += p * V[i, m_prime]
                best_S = S[j, 0]
                Policies[l, i, j] = 0

                for a in range(1, 2 * A[l]):
                    S[j, a] = CX[j, a]
                    if i - a > 0:
                        for m_prime, p in enumerate(P[j]):
                            S[j, a] += p * V[i - a, m_prime]

                    if S[j, a] < best_S:
                        Policies[l, i, j] = a
                        V[i, j] = S[j, a]
                        best_S = S[j, a]


@njit
def update_N(M, Mr, N, N1, states):
    for j, mr in enumerate(Mr):
        for i, m in enumerate(M):
            N[j, i] += ((states[:-1] == mr) * (states[1:] == m)).sum()
    N1 = N.sum(axis=1)


@njit
def update_P(N, N1, M, Mr, P):
    cardM = len(M)
    cardMr = len(Mr)

    for j in range(cardMr):
        I = (N1[j] == 0)
        for i in range(cardM):
            P[j, i] = (N[j, i] + I) / (N1[j] + cardM * I)


class GLOBE:
    """
    Greedy exploitation in Limit Order Book Execution

    """

    def __init__(self,
                 T: int,
                 L: int,
                 W: np.ndarray,
                 lamb: float,
                 eta: float,
                 init_sigma: float,
                 W_max: int,
                 M: list = [],
                 Mr: list = [],
                 K: float = 1.) -> None:
        self.T = T
        self.L = L
        self.W = np.copy(W)
        self.W_remain = np.copy(W)

        self.M = M.copy()
        self.Mr = Mr.copy()
        self.num_of_rounds = W.shape[0]
        self.init_sigma = init_sigma
        self.W_max = W_max
        self.lamb = lamb
        self.eta = eta
        self.K = K

    def estimate_parameters(self,
                            orderbook_bid: dataloader.OnlineData,
                            orderbook_ask: dataloader.OnlineData,
                            ret: np.ndarray = None,
                            rounds_for_est: int = 0):

        m = min(2, rounds_for_est)
        K = self.K

        if ret is None and rounds_for_est > 0:
            ret = np.empty(shape=rounds_for_est)

        for rho in range(m):
            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            pr1 = (F2(1) + F1(1)) / 2
            for l in range(self.T):
                next(orderbook_bid)
                next(orderbook_ask)
            F1 = iter(orderbook_bid).F
            F2 = iter(orderbook_ask).F
            prL = (F2(1) + F1(1)) / 2
            ret[rho] = log(prL / pr1)

        for rho in range(2, rounds_for_est):

            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)

            sigma = sigma_2_rho(ret[:rho]) + 1e-8
            pr1 = (F2(1) + F1(1)) / 2
            pb1 = F1(1)

            for l in range(self.T):
                F2 = next(orderbook_ask)
                F1 = next(orderbook_bid)
                state = int((F1(1) - pb1) / (sigma * K))
                if state not in self.M:
                    self.M.append(state)

            prL = (F2(1) + F1(1)) / 2
            ret[rho] = log(prL / pr1)

    def cumulative_impact(self,
                          orderbook_bid: dataloader.OnlineData,
                          orderbook_ask: dataloader.OnlineData,
                          rounds_for_est: int = 0,
                          model: str = 'basic'):
        ACPR = 0.
        ret = np.empty(shape=self.num_of_rounds + rounds_for_est)
        dt = np.ediff1d(np.linspace(start=0,
                                    stop=self.T,
                                    num=self.L,
                                    endpoint=True,
                                    dtype=int),
                        to_begin=0)
        states = np.empty(shape=self.L)
        self.estimate_parameters(orderbook_bid, orderbook_ask, ret,
                                 rounds_for_est)

        N = np.zeros(shape=(len(self.Mr), len(self.M)))
        N1 = np.zeros(shape=len(self.Mr))
        P = np.empty_like(N)
        Policies = np.empty(shape=(self.L, self.W_max + 1, len(self.M)))

        K = self.K

        M = np.array(self.M)
        Mr = np.array(self.Mr)

        if model == 'basic':
            opt_policy = optimal_policy
        if model == 'plus':
            opt_policy = optimal_policy_plus

        for rho in range(self.num_of_rounds):

            update_P(N, N1, M, Mr, P)

            F2 = next(orderbook_ask)
            F1 = next(orderbook_bid)
            pr1 = (F2(1) + F1(1)) / 2
            pb1 = F1(1)
            prW = pr1 * self.W[rho]
            S = 0
            B = F2(1) - F1(1)

            sigma = sigma_2_rho(ret[:rounds_for_est + rho]) + 1e-8
            A = compute_Al(self.lamb, self.eta, sigma, self.W[rho], self.L)
            opt_policy(A, P, M, self.W_max, pr1, B, sigma, self.W[rho],
                       Policies)

            for l in range(0, self.L):
                for _ in range(dt[l]):
                    next(orderbook_bid)
                    next(orderbook_ask)

                F1 = iter(orderbook_bid).F
                state = int((F1(1) - pb1) / (sigma * K))
                states[l] = state

                m = np.where(M == state)[0][0]

                pi = Policies[l, self.W_remain[rho], m]
                S += iter(orderbook_bid).F(pi)
                self.W_remain[rho] -= pi

            F1 = iter(orderbook_bid).F
            F2 = iter(orderbook_ask).F
            self.W_remain[rho] = 0
            ACPR += (1 - S / (prW))
            prL = (F2(1) + F1(1)) / 2
            ret[rho + rounds_for_est] = log(prL / pr1)
            update_N(M, Mr, N, N1, states)

        return ACPR / self.num_of_rounds

    def reset(self):
        self.__init__(self.T, self.L, self.W, self.lamb, self.eta,
                      self.init_sigma, self.W_max, self.M, self.Mr)
