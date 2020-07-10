function [radii, timestamps, icdd, xpos, ypos] = source_data(n,duration)
    % n - number of bubbles
    % duration - the length in time of data

    % Setup random so its consistent
    rng(1,'philox');

    % Radii (mm)
    mu  = 1.500;
    sig = 0.500;
    radii = normrnd(mu,sig,1,n);

    % Timestamps (ms)
    timestamps =  duration.*rand(1,n);

    % Initial conditions (mm/ms)
    mu  = 5.000;
    sig = 1.000;
    icdd = normrnd(mu,sig,1,n);

    % Bubble positions (mm)
    % Bubbles are placed equally on the surface of the water (2D plane)
    % Bubbles generated in a 8m x 8m area
    % Bubbles outside the measurement area shall be ignored
    min = -4e3; 
    max = +4e3;
    xpos = min + (max-min).*rand(1,n);
    ypos = min + (max-min).*rand(1,n);

end