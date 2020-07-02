% This is the reference data that will be used for my simulated data
% Plunging Jets
[y,fs] = audioread('../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);
t = (0:numel(y)-1)/fs;

% Analyse
[s,f] = cwt(y,fs);
peaks = find_peaks(s,f,t,0.2);

% Plot
plot_scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
plot_peaks(peaks,'r');
    

% Not working - fix the inconsistent peaks matrix

