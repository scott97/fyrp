
wavemngr('del','slti');
wavemngr('add','SOULTI','slti',4,'','soulti_wavelet',[0,40]);

plotwvlt("slti")




% This is the reference data that will be used for my simulated data
% Plunging Jets
[y,fs] = audioread('../DATA/PlungingJets/CircularPlungingJet_x9613135_1.wav', [0.0,1.5]*22050+1);
t = (0:numel(y)-1)/fs;

% Analyse
y = bandpass(y,[500 9000],fs);
scales = 500:50:9000;
[~,s,f] = cwt(y,scales,'slti',1/fs,'scal'); % Using old cwt because new one does not support user defined wavelets
%[s,f] = cwt(y,fs); % The new way
% peaks = find_peaks(s,f,t,0.02);

% Plot
plot_scaleogram(s,f,t,[0 1500],[0 9],sprintf('Plunging jets, wavelet, %dHz',fs));
% plot_peaks(peaks,'r');



% Functions
function plotwvlt(name)
    [psi,xval] = wavefun(name);
    figure
    plot(xval,psi);
    title(name);
    grid on
end