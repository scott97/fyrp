function [y1,y2,y3,t] = generate_spatial_sounds(data, fs, loc1, loc2, loc3)

% Constants
noise_amount = 0.00005;
c = 1500*1000; % Speed of sound seawater (mm/s)

% Data
R_0 = data(1,:) ./ 1000; % Convert from mm to m
t_val = data(2,:) ./ 1000; % Convert from ms to s
dd_ic = data(3,:);
xpos = data(4,:);
ypos = data(5,:);


% Make sounds
t = 0:(1/fs):1.5;
y1 = zeros(1,length(t));
y2 = zeros(1,length(t));
y3 = zeros(1,length(t));

% Hydrophone 1
for i = 1:length(t_val)
    dx = loc1(1) - xpos(i);
    dy = loc1(2) - ypos(i);
    dz = loc1(3);
    dist = sqrt(dx^2 + dy^2 + dz^2); 
    att = 1/(dist^2);
    delay = dist/c;
    d = make_sound(R_0(i), dd_ic(i), t_val(i) + delay, t)*att;
    y1 = y1 + d;
end

% Hydrophone 2
for i = 1:length(t_val)
    dx = loc2(1) - xpos(i);
    dy = loc2(2) - ypos(i);
    dz = loc2(3);
    dist = sqrt(dx^2 + dy^2 + dz^2); 
    att = 1/(dist^2);
    delay = dist/c;
    d = make_sound(R_0(i), dd_ic(i), t_val(i) + delay, t)*att;
    y2 = y2 + d;
end

% Hydrophone 3
for i = 1:length(t_val)
    dx = loc3(1) - xpos(i);
    dy = loc3(2) - ypos(i);
    dz = loc3(3);
    dist = sqrt(dx^2 + dy^2 + dz^2); 
    att = 1/(dist^2);
    delay = dist/c;
    d = make_sound(R_0(i), dd_ic(i), t_val(i) + delay, t)*att;
    y3 = y3 + d;
end

% Amplify
y1 = y1 * 10^10;
y2 = y2 * 10^10;
y3 = y3 * 10^10;

end

