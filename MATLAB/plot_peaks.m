% Finds peaks from a spectrogram/scalogram
function [row,col]=plot_peaks(s,f,t,name,thresh)
% Remove phase info
s = abs(s);

% Find peaks
peaks = imregionalmax(s);
clean = s>thresh; % threshold constant needs to be tuned
BW = peaks .* clean;


% Plot Results
figure
title(name);
xlabel('Time (ms)')
ylabel('Frequency (kHz)')
colorbar
axis tight
hold on

% Scalogram
surface(t*1000,f/1000,s)
shading flat

% Scatter plot of peaks
[row,col] = find(BW);
col = t(col) * 1000;
row = f(row)/1000;
z = ones(size(col)); % for rendering above scalogram

scatter3(col,row,z,'r')
hold off



end




