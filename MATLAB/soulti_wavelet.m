% https://www.researchgate.net/publication/309287880_A_new_wavelet_family_based_on_second-order_LTI-systems

function [psi,t] = soulti_wavelet(lb,ub,n,wname)

    zeta = 0.1; % This would need to be tuned in real life.
    p = 1; %why? I dont know but they say so.
    s = 1; % scaling factor = 1/omega_d.
    T = 0; % shift/offset - tau




    t = linspace(lb,ub,n);

    % variables and functions to make formula simpler
    a = 1-zeta .^ 2;
    x = (t-T) ./ s;

    % wavelet
    psi = ( s .^ (-p) ./ a ) .* exp(-zeta .* x ./ sqrt(a)) .* sin(x) .* ( t-T > 0);

    plot(t,psi)
     
    end