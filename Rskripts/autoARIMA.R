library(readr)
library(forecast)
library(tseries)
library(vars)
library(openxlsx)
library(urca)
library(ggplot2)
library(Metrics)
library(rugarch)
library(rmgarch)
library(quantmod)
library(FAVAR)
# ------------
library(tidyverse)
library(xts)
library(PerformanceAnalytics)


dQI <- read_csv("GitHubRepos/OWModel/ImpactPrediction/DataForImpact/dQI.csv")

speedMI <- dQI$Impact / dQI$DeltaQ

plot(dQI$Impact / dQI$DeltaQ)

min(dQI$DeltaQ)

adf.test(speedMI)

pacf(speedMI)

autoarima <- auto.arima(speedMI)
summary(autoarima)


arima100 <- arima(speedMI, order=c(5,1,0))
summary(arima100)
