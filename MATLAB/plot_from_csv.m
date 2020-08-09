% Import cwt data
s = readmatrix('scaleogram.csv');

% Plot CWT
fs = 44100;
t = (0:length(s)-1)/fs;
f = 1000:10:9000;
plot_utils.scaleogram(s,f,t,[0 100],[1 9],'my cwt (rust impl)');