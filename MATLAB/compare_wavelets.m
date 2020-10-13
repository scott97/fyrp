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

% Define wavelets
morlet_real = @(t) exp(-(tau/5 .* t) .^ 2 ./2) .* cos(tau .* t);
morlet_cpx = @(t) exp(-(tau/5 .* t) .^ 2 ./2) .* exp(j*tau .* t);

zeta = 0.02;
k = 1-zeta^2;
soulti_real = @(t) 1/k .* exp(-zeta/sqrt(k) * tau .* t) .* sin(tau .* t) .* (t>0);
soulti_cpx = @(t) 1/k .* exp(-zeta/sqrt(k) * tau .* t) .* exp(j*tau .* t) .* (t>0);

% Plot wavelets
figure
t = -3:.01:3;
plot3(t,real(morlet_cpx(t)),imag(morlet_cpx(t)))
xlabel('Time');
ylabel('Real component');
zlabel('Imaginary component');
title('Morlet wavelet, with a frequency of 1Hz')
grid on

figure
t = -3:.01:3;
plot(t,real(morlet_cpx(t)))
hold on
plot(t,imag(morlet_cpx(t)))
hold off
xlabel('Time');
ylabel('Magnitude');
title('Morlet wavelet, with a frequency of 1Hz')
legend('Real component','Imaginary component')
grid on

figure
t = .01:.01:50;
plot3(t,real(soulti_cpx(t)),imag(soulti_cpx(t)))
xlabel('Time');
ylabel('Real component');
zlabel('Imaginary component');
title('Laplace wavelet (ζ=0.05), with a frequency of 1Hz')
grid on

figure
t = 0:.01:50;
plot(t,real(soulti_cpx(t)))
hold on
plot(t,imag(soulti_cpx(t)))
hold off
xlabel('Time');
ylabel('Magnitude');
title('Laplace wavelet (ζ=0.05), with a frequency of 1Hz')
legend('Real component','Imaginary component')
grid on


% Get data
[y,t] = cached_one_hydrophone(source_data, fs, loc,1.5);
y = bandpass(y,[100 9000],fs);

% Plot CWT
t = (0:numel(y)-1)/fs;

% Morlet Real
[s,f] = bubble_analysis.cwt(y, morlet_real, [-3 3], 1000:50:9000, fs);
% peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (Morlet Real)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% Morlet Complex
[s,f] = bubble_analysis.cwt(y, morlet_cpx, [-9 9], 1000:50:9000, fs);
s=abs(s);
% peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'my cwt (Morlet Complex)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% Soulti Real
[s,f] = bubble_analysis.cwt(y, soulti_real, [0 10], 1000:50:9000, fs);
% peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'SOULTI wavelet (ζ=0.02)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% Soulti Complex
[s,f] = bubble_analysis.cwt(y, soulti_cpx, [0 50], 1000:50:9000, fs);
s=abs(s);
% peaks = bubble_analysis.find_peaks(s,f,t,0.12);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'Laplace wavelet (ζ=0.02)');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');

% MATLAB built-in
[s,f] = cwt(y,fs);
% peaks = bubble_analysis.find_peaks(s,f,t,0.09);
plot_utils.scaleogram(s,f,t,[0 1500],[1 9],'matlab cwt');
% plot_utils.peaks(peaks,'r');
% plot_utils.peaks(comparison_data,'g');


