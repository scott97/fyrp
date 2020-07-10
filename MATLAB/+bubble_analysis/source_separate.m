function y = source_seperate(t,fs,y1,y2,y3,pos,loc1,loc2,loc3)
    % Distances (mm)
    dx = loc1(1)-pos(1);
    dy = loc1(2)-pos(2);
    dz = loc1(3)-pos(3);
    d1 = sqrt(dx^2+dy^2+dz^2);

    dx = loc2(1)-pos(1);
    dy = loc2(2)-pos(2);
    dz = loc2(3)-pos(3);
    d2 = sqrt(dx^2+dy^2+dz^2);

    dx = loc3(1)-pos(1);
    dy = loc3(2)-pos(2);
    dz = loc3(3)-pos(3);
    d3 = sqrt(dx^2+dy^2+dz^2);

    % Constants
    c = 1500e3; % Speed of sound in seawater (mm/s)


    y1 = interp1(t,y1,t + d1/c,'spline',0);
    y2 = interp1(t,y2,t + d2/c,'spline',0);
    y3 = interp1(t,y3,t + d3/c,'spline',0);

    y = y1 + y2 + y3;
end