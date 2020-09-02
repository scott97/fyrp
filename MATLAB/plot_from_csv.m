function plot_from_csv(idx,chunk_duration)
    % duration in milliseconds
    % Import cwt data
    file = sprintf('scaleogram%d.csv', idx);
    s = csvread(file);

    % Plot CWT
    fs = 44100;

    [h,l] = size(s);
    t = (0:l-1)/fs + (chunk_duration*1e-3*(idx-1));
    f = linspace(1000,9000,h);
    
    plot_utils.scaleogram(s,f,t,[min(t)*1000 max(t)*1000],[1 9],'my cwt (rust impl)');
end
