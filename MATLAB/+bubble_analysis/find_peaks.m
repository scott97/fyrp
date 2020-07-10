% Finds peaks from a spectrogram/scalogram
function data=find_peaks(s,f,t,thresh)

% Remove phase info
s = abs(s);

% Find peaks
peaks = imregionalmax(s);
clean = s>thresh; % threshold constant needs to be tuned
BW = peaks .* clean;

[row,col] = find(BW);

col = t(col)*1000;
row = f(row)/1000;

data = [row';col];

end

