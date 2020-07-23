% MATLAB testing my wavelet implementation

% Plunging Jets
[y,fs] = audioread('../../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);


% Constants
tau = 2*pi

% Define wavelets
morlet = @(t) exp(-(tau/5 .* t) .^ 2 ./2) .* cos(tau .* t);
zeta = 0.1;
k = 1-zeta^2;
soulti = @(t) 1/k .* exp(-zeta/k * tau .* t) .* sin(tau .* t) .* (t>0);

% Show wavelets
figure
t = -3:.01:3;
plot(t,morlet(t))

figure
t = 0:.01:10;
plot(t,soulti(t))

% Plot CWT
t = (0:numel(y)-1)/fs;

% Morlet
[s,f] = bubble_analysis.cwt(y, morlet, [-3 3], 1000:50:9000, 22050);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (morlet)');

% SOULTI
[s,f] = bubble_analysis.cwt(y, soulti, [0 10], 1000:50:9000, 22050);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (SOULTI)');

% MATLAB built-in
[s,f] = cwt(y,fs);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'matlab cwt');



