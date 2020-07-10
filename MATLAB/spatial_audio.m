% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@misc_utils.source_data);
cached_generate_spatial_sounds = memoize(@generate_audio.three_hydrophone);


% Get source data
[radii, timestamps, icdd, xpos, ypos] = cached_source_data(1000,1500);
zone = sqrt(xpos.^2 + ypos.^2) < 1000; % I only want to measure bubbles in a 1m radius of the surface

plot_utils.spatial_data(radii, timestamps, icdd, xpos, ypos, zone);

source_data = [
	radii;
    timestamps;
    icdd;
    xpos;
    ypos;
    zone;
];


% Data for comparison purposes
k = 1/(2*pi) * sqrt(3*1.4*101325/1000);
comparison_data = [ k ./ radii(zone); timestamps(zone) ];

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
plot_utils.hydrophone_array(loc1,loc2,loc3);

% Get data
fs = 88200;
[y1,y2,y3,t] = cached_generate_spatial_sounds(source_data, fs, loc1, loc2, loc3);

% Analyse
y = bubble_analysis.source_separate(t,fs,y1,y2,y3,[0;0;0],loc1,loc2,loc3);


y = bandpass(y,[500 9000],fs);
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);

% Plot
plot_utils.scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');
