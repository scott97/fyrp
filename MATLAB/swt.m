function s = swt(y,wavelet,offet_interval)
    % Slow wavelet transform - its probably not very good.
    disp(length(y))

    for offset = 1:offet_interval:length(y)
        for scale = 1:30
            total = 0;
            for i = 1:length(y)
                total += sum(y(i) .* wavelet((i-offset)/scale));
            end
            s(offset,scale) = total;
        end
        disp(offset)
    end
end

