% Plunging Jets
[y,fs] = audioread('../../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);
y = y(1:2000);

% Analyse
morlet = @(t) exp(-t ^ 2 /2) * cos(5*t); 
s = swt(y,morlet,100);

% Plot
surf(s);