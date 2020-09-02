% Get source data
[radii, timestamps, icdd, xpos, ypos] = misc_utils.source_data(1100,10000); % 1,100 bubbles in 10,000 milliseconds
zone = sqrt(xpos.^2 + ypos.^2) < 1000; % I only want to measure bubbles in a 1m radius of the surface

% plot_utils.spatial_data(radii,timestamps,icdd,xpos,ypos,zone)

source_data = [
	radii;
    timestamps;
    icdd;
    xpos;
    ypos;
    zone;
];

% Hydrophone array layout
% Side view
% ~~~~~~o~O~~o~~oo~~~~~~O~~~~Oo~~~o~~~~~~  ← surface
%                                          ↕ depth
%                   H1                     ← hydrophone

% Test at 400mm below surface
loc = [0;0;-400];

% Constants
fs = 44100;

% Get data
[y,~] = generate_audio.one_hydrophone(source_data, fs, loc, 10);

% Normalise data
y = ( y - min(y) ) / ( max(y) - min(y) );
y = 2 .* (y - 0.5);

% Export audio
audiowrite('data.wav',y,fs);

% Export source data
source_data = [
	radii(zone);
    timestamps(zone);
];
csvwrite('source_data.csv',source_data);