function scatter_plot_from_csv(from, to, chunk_duration)
figure
title('Algorithm output (+) vs ideal case (o)');
xlabel('Time of formation (ms)')
ylabel('Radius (mm)')
grid on;

% Import & plot theoretical values
csv = csvread('../tmp/source_data.csv');
plot_utils.peaks(csv,'k');

% Import & plot algorithm output
hold on
for idx = from:to
    file = sprintf('../tmp/bubbles%d.csv', idx);
    disp(file)
    colours = ['r+';'g+';'b+';'m+'];
    
    chunk_colour = colours(mod(idx,4)+1,:);
    disp(chunk_colour)
    try
        csv = csvread(file);
        csv(2,:) = csv(2,:) + chunk_duration * (idx-1);
        scatter(csv(2,:),csv(1,:),chunk_colour)
    end
end
hold off
