function [s,f] = cwt(y,wvlt_fn,wvlt_bounds,f,fs)
    % y - signal
    % wvlt_fn - wavelet function of form @(t), must have a frequency of 1Hz!
    % wvlt_bounds - an array [min,max], where the wavelet function is equal to zero below min and above max
    % f - an array of all the frequencies you would like data for
    % fs - sample rate

    for i = 1:length(f)
        scale = 1/f(i);
        t = (wvlt_bounds(1)*scale):(1/fs):(wvlt_bounds(2)*scale);
        wv = wvlt_fn(t/scale);
        row = conv(y,fliplr(wv)) .* (1/sqrt(scale));
        s(i,:) = row(length(wv):end);
        % For morlet, replace with
        % s(i,:) = row(length(wv)-ceil(length(wv)/2):end-ceil(length(wv)/2));
    end
    f = f'; % To match matlab's implementation
end

