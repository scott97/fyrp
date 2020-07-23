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
% ~~~~~~o~O~~o~~oo~~~~~~O~~~~Oo~~~o~~~~~~  ← surface
%                                          ↕ depth
%                   H1                     ← hydrophone

% Test at 400mm below surface
loc = [0;0;-400];

% Constants
fs = 44100;
tau = 2*pi;

% Define wavelets
morlet = @(t) exp(-(tau/5 .* t) .^ 2 ./2) .* cos(tau .* t);
zeta = 0.1;
k = 1-zeta^2;
soulti = @(t) 1/k .* exp(-zeta/k * tau .* t) .* sin(tau .* t) .* (t>0);

% Get data
[y,t] = cached_one_hydrophone(source_data, fs, loc);
y = bandpass(y,[100 9000],fs);

% Plot CWT
t = (0:numel(y)-1)/fs;

% Morlet
[s,f] = bubble_analysis.cwt(y, morlet, [-3 3], 1000:50:9000, fs);
% peaks = bubble_analysis.find_peaks(s,f,t,0.09);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (morlet)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% SOULTI
[s,f] = bubble_analysis.cwt(y, soulti, [0 10], 1000:50:9000, fs);
% peaks = bubble_analysis.find_peaks(s,f,t,0.09);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (SOULTI)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% MATLAB built-in
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'matlab cwt');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');


