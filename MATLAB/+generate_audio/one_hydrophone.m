function [y,t] = one_hydrophone(data, fs, loc)

    % Constants
    c = 1500e3; % Speed of sound seawater (mm/s)

    % Data
    R_0 = data(1,:) ./ 1000; % Convert from mm to m
    t_val = data(2,:) ./ 1000; % Convert from ms to s
    dd_ic = data(3,:);
    xpos = data(4,:);
    ypos = data(5,:);


    % Make sounds
    t = 0:(1/fs):1.5;
    y = zeros(1,length(t));

    for i = 1:length(t_val)
        dx = loc(1) - xpos(i);
        dy = loc(2) - ypos(i);
        dz = loc(3);
        dist = sqrt(dx^2 + dy^2 + dz^2); 
        att = 1/(dist^2);
        delay = dist/c;
        d = generate_audio.make_sound(R_0(i), dd_ic(i), t_val(i) + delay, t)*att;
        y = y + d;  
    end

    % Add some signal noise to simulate electrical noise
    n = 0.05*max(y);
    y = y + randn([1,length(y)]) * n;

end

