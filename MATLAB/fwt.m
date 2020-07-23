function [s,f] = fwt(y,wvlt_fn,wvlt_bounds,bands,fs)
    % Wavelet functions must have a frequency of 1Hz!

    for i = 1:length(bands)
        scale = bands(i);
        t = (wvlt_bounds(1)/scale):(1/fs):(wvlt_bounds(2)/scale);
        wv = wvlt_fn(t*scale);
        row = conv(y,wv) .* (1/sqrt(1/scale));
        s(i,:) = row(1:length(y));
    end
    f = bands;
end

