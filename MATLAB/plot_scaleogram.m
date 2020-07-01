% Finds peaks from a spectrogram/scalogram
function [row,col]=plot_scaleogram(s,f,t,name)
    % Remove phase info
    s = abs(s);

    % Plot Results
    figure
    title(name);
    xlabel('Time (ms)')
    ylabel('Frequency (kHz)')
    colorbar
    axis tight

    % Scalogram
    surface(t*1000,f/1000,s)
    shading flat

end




