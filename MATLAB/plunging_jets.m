% MATLAB testing my wavelet implementation

% Plunging Jets
[y,fs] = audioread('../../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);

% Define wavelet
morlet = @(t) exp(-((2*pi/5) .* t) .^ 2 ./2) .* cos((2*pi).*t);

% Show wavelet
t = -3:.01:3;
plot(t,morlet(t))

% CWT
[s,f] = fwt(y, morlet, [-3 3], 1000:50:9000, 22050);
[s2,f2] = cwt(y,fs);

% Plot
t = (0:numel(y)-1)/fs;
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt');
plot_utils.scaleogram(s2,f2,t,[0 1500],[1 9],'matlab cwt');
