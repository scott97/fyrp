function plot_peaks(peaks,colour)
    hold on
    z = ones(length(peaks),1); % for rendering above
    scatter3(peaks(:,2),peaks(:,1),z,colour)
    hold off
end