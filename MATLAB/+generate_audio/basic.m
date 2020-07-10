function [y,t] = basic(data,fs)

    % Data
    R_0 = data(1,:) ./ 1000; % Convert from mm to m
    t_val = data(2,:) ./ 1000; % Convert from ms to s
    dd_ic = data(3,:);


    % Make sounds
    t = 0:(1/fs):1.5;
    y = zeros(1,length(t));


    for i = 1:length(t_val)
        d = generate_audio.make_sound(R_0(i), dd_ic(i), t_val(i), t);
        y = y + d;
    end

    % Add some signal noise to simulate electrical noise
    n = 0.05*max(y);
    y = y + randn([1,length(y)]) * n;

end

