from statsmodels.tsa.stattools import adfuller 
from numpy import log 
import numpy as np
import pandas as pd 
from scipy.optimize import least_squares

class GOWModel:
    '''
    The Implementation of the Generalized Obyzhaeva-Wang Market Impact model.
    We predict MI in the form I(t_{i+1}) = \rho^{t_{i+1} - t_{i}} I(t_{i}) + \lamda \Delta Q,
    where I is MI and \Delta Q is a change of our position.
    Details in A3 of 'Handbook of Market Impact Modelling'
    '''
    def __init__(self, lambd=1., rho=0.5):
        self.lambd = lambd
        self.rho = rho

    def fitLS(self, # This method fits the model using least_squares from scipy.optimize
            MI: np.array, # Market Impact array 
            dQ: np.array,  # Changes of position array
            T: np.array # The array of moments of position changes 
           ):
        '''
        This method fits the model using least_squares from scipy.optimize
        '''
        def fun(x, mi: np.array, mi_prev: np.array, dq: np.array, dt: np.array): # residual function to optimize - see the scipy.optimize.least_squares docs
            return np.power(x[0], dt) * mi_prev + x[1] * dq - mi
        
        res_lsq = least_squares(fun, np.array([self.rho, self.lambd]), args=(MI[1:], MI[:-1], dQ[:-1], np.diff(T)))

        self.rho = res_lsq.x[0]
        self.lambd = res_lsq.x[1]
        self.result = res_lsq
        self.lastMI = MI[-1] 

    def predictWithMI(self, 
                MI: np.array,
                dQ: np.array, # Changes of position array
                T: np.array # The array of moments of position changes
                ):
        return np.power(self.rho, np.diff(T)) * MI[:-1] + self.lambd * dQ[:-1]
    
    def predict(self, 
                dQ: np.array, # Changes of position array
                T: np.array, # The array of moments of position changes
                ):
        ans = np.array([])
        for i in range(len(np.diff(T))):
            pmi = np.power(self.rho, np.diff(T)[i]) * self.lastMI + self.lambd * dQ[i]
            np.append(ans, pmi)
            self.lastMI = pmi

        return ans

