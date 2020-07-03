% Solving 2ODE's is slow so this just uses an algebraic expression.
function [t,d] = make_sound(R_0, ddot_0, t)

    % Constants
    K_p = 1.4; % 1.4
    p_0 = 1000; % 1000 kg/m^3
    P_0 = 101325; % 1 atm or 101,325 Pa
    b = 0.6; % (need to research typical values)

    % Calculated Constants
    a = R_0;
    b;
    c = (3 * K_p * P_0) / (p_0 * R_0);

    % More Calculated Constants
    k = b/(2*a);
    w = sqrt( (c/a) - (k^2) );
    M = ddot_0/w;


    % Evaluate
    d = M * exp(-k .* t) .* sin(w .* t);
    d = d';
    t = t';

end


