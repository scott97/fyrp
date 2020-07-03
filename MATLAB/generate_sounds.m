function [y,t] = generate_sounds(data,fs)

% Constants
noise_amount = 0.00005;

% Data
R_0 = data(1,:) ./ 1000; % Convert from mm to m
t_val = data(2,:) ./ 1000; % Convert from ms to s
dd_ic = data(3,:);


% Make sounds
t = 0:(1/fs):1.5;
y = zeros(1,length(t));


for i = 1:length(t_val)
    d = make_sound(R_0(i), dd_ic(i), t_val(i), t);
    y = y + d;
end

% Add some signal noise
noise = randn([1,length(y)]) * noise_amount;
y = y + noise;

% Amplify
y = y * 10^4 * 0.18;

end

