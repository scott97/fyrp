% Solving 2ODE's is slow so this just uses an algebraic expression.
function d = make_sound(R_0, T, t)

    % Bubbles oscillate according to this differential equation:
    % eq1: R_0 * δdotdot + b * δdot + (3 * K_p * P_0) / (ρ_0 * R_0) * δ = 0
    % 
    % A commonly used formula for oscillations is:
    % δdotdot + (2*ζ*ωn)*δdot + (ωn^2)*δ = 0
    % 
    % This has a solution:
    % eq2: δ = ωn / sqrt(1-ζ^2) * e^(-ζ*ωn*t) * sin(ωd*t)
    % where ωd = ωn * sqrt(1-ζ^2)
    % 
    % Divide eq1 by R_0.
    % δdotdot + (b/R_0)*δdot + (3 * K_p * P_0) / (ρ_0 * R_0^2) * δ 
    % 
    % Therefore:
    % ωn^2 = (3 * K_p * P_0) / (ρ_0 * R_0^2)
    % ωn = sqrt( (3 * K_p * P_0) / (ρ_0 * R_0^2) )

    % Constants
    K_p = 1.4; % 1.4
    p_0 = 1000; % 1000 kg/m^3
    P_0 = 101325; % 1 atm or 101,325 Pa

    % Calculate zeta from radius.
    % b is not known, but ζ is.
    % Known points (we interpolate for in between)
    % ζ = 0.02 for 1 mm and ζ = 0.013 for 5 mm
    z_p = [0.02, 0.013];
    r_p = [1e-3, 5e-3];

    z = z_p(1) + ( R_0 - r_p(1) ) * ( z_p(2) - z_p(1) ) / ( r_p(2) - r_p(1) );

    % Calculating ωn and ωd.
    wn = sqrt( (3 * K_p * P_0) / (p_0 * R_0^2) );
    wd = wn * sqrt(1-z^2);

    % Evaluate
    % Signal amplitude is proportional to radius, so the first term is R_0
    d = R_0 .* exp(-z .* wn .* (t-T)) .* sin(wd .* (t-T)) .* ((t-T) > 0);

    % The exp function is causing infinities or NaNs for large negative numbers of (t-T)
    % These matrix elements should be zero because they occur before T.
    % This patches them up as zero.
    TF = isnan(d);
    d(TF) = 0;

end


