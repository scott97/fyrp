function plot_hydrophone_array(loc1,loc2,loc3)
    figure;
    w = (-10:10) .* 100;
    [X,Y] = meshgrid(w,w);
    Z = zeros(size(X));
    s = surf(X,Y,Z);
    s.FaceColor = 'none';

    hold on
    X = [loc1(1),loc2(1),loc3(1)];
    Y = [loc1(2),loc2(2),loc3(2)];
    Z = [loc1(3),loc2(3),loc3(3)];
    scatter3(X,Y,Z);
    hold off
end