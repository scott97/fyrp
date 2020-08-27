function plot_from_csv(idx)
    % Import cwt data
    file = sprintf('scaleogram%d.csv', idx)
    s = csvread(file);

    % Plot CWT
    fs = 44100;

    [h,l] = size(s)
    t = (0:l-1)/fs;
    f = linspace(1000,9000,h);
    plot_utils.scaleogram(s,f,t,[0 100],[1 9],'my cwt (rust impl)');
end
