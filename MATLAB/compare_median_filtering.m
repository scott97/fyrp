% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@misc_utils.source_data);
cached_one_hydrophone = memoize(@generate_audio.one_hydrophone);

% Get source data
[radii, timestamps, icdd, xpos, ypos] = cached_source_data(1000,1500);
zone = sqrt(xpos.^2 + ypos.^2) < 1000; % I only want to measure bubbles in a 1m radius of the surface

% plot_utils.spatial_data(radii, timestamps, icdd, xpos, ypos, zone);

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
% ~~~~~~o~O~~o~~oo~~~~~~O~~~~Oo~~~o~~~~~~  ← surface
%                                          ↕ depth
%                   H1                     ← hydrophone

% Test at 400mm below surface
loc = [0;0;-400];

% Constants
fs = 44100;
tau = 2*pi;

% Define wavelet
zeta = 0.02;
k = 1-zeta^2;
soulti_cpx = @(t) 1/k .* exp(-zeta/k * tau .* t) .* exp(j*tau .* t) .* (t>0);


% Get data
[y,t] = cached_one_hydrophone(source_data, fs, loc,1.5);
y = bandpass(y,[100 9000],fs);

% Plot CWT
t = (0:numel(y)-1)/fs;


[s,f] = bubble_analysis.cwt(y, soulti_cpx, [0 50], 1000:50:9000, fs);
s=abs(s);


% No Filter
peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'No Filter');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% 2D Median Filter (3x3)
s2 = medfilt2(s, [3 3]);
peaks = bubble_analysis.find_peaks(s2,f,t,0.12);
plot_utils.scaleogram(s2,f,t,[0 1500],[1 9],'2D Median Filter (3x3)');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% 2D Median Filter (6x6)
s3 = medfilt2(s, [12 12]);
peaks = bubble_analysis.find_peaks(s3,f,t,0.12);
plot_utils.scaleogram(s3,f,t,[0 1500],[1 9],'2D Median Filter (6x6)');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% 1D filter of input (3)
[y,t] = cached_one_hydrophone(source_data, fs, loc,1.5);
y = bandpass(y,[100 9000],fs);
y = medfilt1(y,3)

[s,f] = bubble_analysis.cwt(y, soulti_cpx, [0 50], 1000:50:9000, fs);
s=abs(s);

peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'1D Median Filter (3)');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');

% 1D filter of input (6)
[y,t] = cached_one_hydrophone(source_data, fs, loc,1.5);
y = bandpass(y,[100 9000],fs);
y = medfilt1(y,6)

[s,f] = bubble_analysis.cwt(y, soulti_cpx, [0 50], 1000:50:9000, fs);
s=abs(s);

peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'1D Median Filter (6)');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');