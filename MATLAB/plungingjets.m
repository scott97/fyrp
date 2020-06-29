% This is the reference data that will be used for my simulated data
% Plunging Jets
[y,fs] = audioread('../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);

% Wavelet
[s,f] = cwt(y,fs);
t = (0:numel(y)-1)/fs;
[f_khz, t_ms] = plot_peaks(s,f,t,'Plunging Jets, wavelet',0.025);

