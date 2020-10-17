function scatter_plot_from_csv_alt()

video_csv = '../rust/video/video.csv';
audio_csv = '../rust/video/1-thresh750-radius2to6-bandwidth50-maxiter40.csv';

video_lag_time= 35;
audio_lag_time = 200; % 200ms is equal to the length of a single segment.

figure
title('Bubble identifications on experimental data');
xlabel('Time of formation (ms)')
ylabel('Radius (mm)')
grid on;

% Import & plot video identifications
csv = csvread(video_csv);
csv(2,:) = csv(2,:) - video_lag_time;
plot_utils.peaks(csv,'k');

% Import & plot audio identifications
hold on
csv = csvread(audio_csv);
csv(2,:) = csv(2,:) - audio_lag_time;
plot_utils.peaks(csv,'r+');
hold off


legend('Video identifications','Audio identifications')

xlim([0,1e3*60*2]);
ylim([0,8]);