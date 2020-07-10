% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@misc_utils.source_data);
cached_conical = memoize(@generate_audio.conical_beam_hydrophone);

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
%                    H                     ← hydrophone test plots
% 

% Test at 400mm depth
loc = [0;0;-400];

% Constants
fs = 44100;

% Get audio
[y,t] = cached_conical(source_data, fs, loc);

% Analyse
y = bandpass(y,[500 9000],fs);
[s,f] = cwt(y,fs);
peaks = bubble_analysis.find_peaks(s,f,t,0.09);

% Plot
plot_utils.scaleogram(s,f,t,[0 1500],[0 9],'Conical beam hydrophone - depth 400mm');
plot_utils.peaks(peaks,'r');
plot_utils.peaks(comparison_data,'g');
