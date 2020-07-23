% Plunging Jets
[y,fs] = audioread('../../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);

% Analyse
morlet = @(t) exp(-((2*pi/5) .* t) .^ 2 ./2) .* cos((2*pi).*t);
t = -3:.01:3;
plot(t,morlet(t))

frequencies = 100:50:9000;


[s,f] = fwt(y, morlet, [-3,3], frequencies, 22050);

[s2,f2] = cwt(y,fs);

size(s)
size(f)

% Plot
t = (0:numel(y)-1)/fs;
plot_utils.scaleogram(s,f,t,[0 1500],[0 9],'my cwt');
plot_utils.scaleogram(s2,f2,t,[0 1500],[0 9],'matlab cwt');

