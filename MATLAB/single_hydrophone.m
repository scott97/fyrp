% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@misc_utils.source_data);
cached_one_hydrophone = memoize(@generate_audio.one_hydrophone);

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
% ~~~~~~o~O~~o~~oo~ H1 ~O~~~~Oo~~~o~~~~~~  ← surface
%                                          ↕ depth
%                   H2                     ← hydrophone test plots
% 
%                   H3

% Test at three depths below surface (mm)
loc1 = [0;0;0];
loc2 = [0;0;-400];
loc3 = [0;0;-800];
plot_utils.hydrophone_array(loc1,loc2,loc3);

% Constants
fs = 44100;

% Location 1
[y,t] = cached_one_hydrophone(source_data, fs, loc1);
y = bandpass(y,[500 9000],fs);
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);

plot_utils.scaleogram(s,f,t,[0 1500],[0 9],'Single hydrophone - depth 0mm');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% Location 2
[y,t] = cached_one_hydrophone(source_data, fs, loc2);
y = bandpass(y,[500 9000],fs);
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);

plot_utils.scaleogram(s,f,t,[0 1500],[0 9],'Single hydrophone - depth 400mm');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% Location 3
[y,t] = cached_one_hydrophone(source_data, fs, loc3);
y = bandpass(y,[500 9000],fs);
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);

plot_utils.scaleogram(s,f,t,[0 1500],[0 9],'Single hydrophone - depth 800mm');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');
