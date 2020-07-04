function plot_spatial_data(radii, timestamps, icdd, xpos, ypos)
    figure
    scatter3(xpos, ypos, timestamps, radii .* 25, icdd); % .* 25 is to improve legibility
    axis tight
    title("Spatial data - colour: icdd, size: radius")
    xlabel("X position on surface (mm)")
    ylabel("Y position on surface (mm)")
    zlabel("time (ms)")
    colorbar;
end