function spatial_data(radii, timestamps, icdd, xpos, ypos, zone)
    figure
    scatter3(xpos(zone), ypos(zone), timestamps(zone), radii(zone) .* 25, icdd(zone),'o'); % .* 25 is to improve legibility
    axis tight
    title("Spatial data - colour: icdd, size: radius")
    xlabel("X position on surface (mm)")
    ylabel("Y position on surface (mm)")
    zlabel("time (ms)")
    colorbar;
    hold on
    scatter3(xpos(~zone), ypos(~zone), timestamps(~zone), radii(~zone) .* 25, icdd(~zone),'+');
    hold off
end