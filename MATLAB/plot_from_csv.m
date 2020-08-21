% Import cwt data
s = csvread('scaleogram.csv');

% Plot CWT
fs = 44100;

[h,l] = size(s)
t = (0:l-1)/fs;
f = linspace(1000,9000,h);
plot_utils.scaleogram(s,f,t,[0 100],[1 9],'my cwt (rust impl)');