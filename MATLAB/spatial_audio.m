% TODO - finish this
% beamforming stuff

% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@source_data);
cached_generate_spatial_sounds = memoize(@generate_spatial_sounds);


% Get source data
[radii, timestamps, icdd, xpos, ypos] = cached_source_data(300,1500);
plot_spatial_data(radii, timestamps, icdd, xpos, ypos);

source_data = [
	radii;
    timestamps;
    icdd;
    xpos;
    ypos;
];


% Data for comparison purposes
k = 1/(2*pi) * sqrt(3*1.4*101325/1000);
comparison_data = [ k ./ radii; timestamps ];

% Hydrophone array layout
% Side view
% ~~~~~~o~O~~o~~oo~~~~~O~~~~~Oo~~~o~~~~~~  ← surface
%                                          ↕ depth
%                 H H H                    ← hydrophones
% 
% Top view
%                   H
%                 H   H

z = -800; % depth below surface (mm)
d = 150; % distance from centre (mm)
loc1 = [d*cos(0);d*sin(0);z];
loc2 = [d*cos(2*pi/3);d*sin(2*pi/3);z];
loc3 = [d*cos(4*pi/3);d*sin(4*pi/3);z];
plot_hydrophone_array(loc1,loc2,loc3);

% Get data
fs = 88200;
[y1,y2,y3,t] = cached_generate_spatial_sounds(source_data, fs, loc1, loc2, loc3);

% Analyse
y1 = bandpass(y1,[500 9000],fs);
[s,f] = cwt(y1,fs);
peaks = find_peaks(s,f,t,0.2);

% Plot
plot_scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
plot_peaks(peaks,'r');
plot_peaks(comparison_data,'g');

% Analyse
y2 = bandpass(y2,[500 9000],fs);
[s,f] = cwt(y2,fs);
peaks = find_peaks(s,f,t,0.2);

% Plot
plot_scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
plot_peaks(peaks,'r');
plot_peaks(comparison_data,'g');

% Analyse
y3 = bandpass(y3,[500 9000],fs);
[s,f] = cwt(y3,fs);
peaks = find_peaks(s,f,t,0.2);

% Plot
plot_scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
plot_peaks(peaks,'r');
plot_peaks(comparison_data,'g');