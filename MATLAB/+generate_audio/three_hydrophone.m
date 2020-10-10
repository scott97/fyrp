function [y1,y2,y3,t] = three_hydrophone(data, fs, loc1, loc2, loc3)

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

    for i = 1:length(t_val)

        % Hydrophone 1
        dx = loc1(1) - xpos(i);
        dy = loc1(2) - ypos(i);
        dz = loc1(3);
        dist = sqrt(dx^2 + dy^2 + dz^2); 
        att = 1/dist;
        delay = dist/c;
        d = generate_audio.make_sound(R_0(i), t_val(i) + delay, t)*att;
        y1 = y1 + d;

        % Hydrophone 2
        dx = loc2(1) - xpos(i);
        dy = loc2(2) - ypos(i);
        dz = loc2(3);
        dist = sqrt(dx^2 + dy^2 + dz^2); 
        att = 1/dist;
        delay = dist/c;
        d = generate_audio.make_sound(R_0(i), t_val(i) + delay, t)*att;
        y2 = y2 + d;

        % Hydrophone 3
        dx = loc3(1) - xpos(i);
        dy = loc3(2) - ypos(i);
        dz = loc3(3);
        dist = sqrt(dx^2 + dy^2 + dz^2); 
        att = 1/dist;
        delay = dist/c;
        d = generate_audio.make_sound(R_0(i), t_val(i) + delay, t)*att;
        y3 = y3 + d;
    end

    % Add some signal noise to simulate electrical noise
    n = 0.05*max(y1);
    y1 = y1 + randn([1,length(y1)]) * n;
    y2 = y2 + randn([1,length(y2)]) * n;
    y3 = y3 + randn([1,length(y3)]) * n;

end

