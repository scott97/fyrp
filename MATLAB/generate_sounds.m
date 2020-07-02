function [y,t] = generate_sounds(data,fs)

% Constants
noise_amount = 0.00005;

% Data
R_0 = data(1,:) ./ 1000; % Convert from mm to m
t_val = data(2,:) ./ 1000; % Convert from ms to s
dd_ic = data(3,:);


% Make sounds
t = 0:(1/fs):1.5;
y = zeros(length(t),1);


for i = 1:length(t_val)
    [~,d] = make_sound(R_0(i), dd_ic(i),0:(1/fs):1.5);
    idx = floor(t_val(i)*fs); % times or divide?
    shifted = circshift(d,idx,1);
    y = y + shifted(1:length(y));
end

% Add some background noise
noise = randn([length(y),1]) * noise_amount;
y = y + noise;

% Amplify
y = y * 10^4 * 0.18;

end

