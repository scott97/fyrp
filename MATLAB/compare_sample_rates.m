% Memoized funcs
% These are nolonger super slow but they are still worth cacheing
cached_source_data = memoize(@misc_utils.source_data);
cached_generate_sounds = memoize(@generate_audio.basic);


% Get source data
[radii, timestamps, icdd, xpos, ypos] = cached_source_data(300,1500);

source_data = [
	radii;
    timestamps;
    icdd;
];


% Data for comparison purposes
k = 1/(2*pi) * sqrt(3*1.4*101325/1000);
comparison_data = [ k ./ radii; timestamps ];


% Get data
max_sample_rate = 88200;
[y88k,~] = cached_generate_sounds(source_data, max_sample_rate);

% Analyse and plot
for i = 0:2
    fs = max_sample_rate / (2^i); 
    
    % Downsample
    y = y88k(1:(2^i):end);
    t = (0:numel(y)-1)/fs;

    % Analyse
    y = bandpass(y,[500 9000],fs);
    [s,f] = cwt(y,fs);
    peaks = bubble_analysis.find_peaks(s,f,t,0.09);

    % Plot
    plot_utils.scaleogram(s,f,t,[0 1500],[0 9],sprintf('Sythesised by me, wavelet, %dHz',fs));
    plot_utils.peaks(peaks,'r');
    plot_utils.peaks(comparison_data,'g');

end