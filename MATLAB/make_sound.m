% https://au.mathworks.com/matlabcentral/answers/66301-solve-a-second-order-differential-equation

function [t,d] = make_sound(R_0, ddot_0, tspan)

% Constants
K_p = 1.4; % 1.4
p_0 = 1000; % 1000 kg/m^3
P_0 = 101325; % 1 atm or 101,325 Pa
b = 0.6; % (need to research typical values)

% Calculated Constants
a = R_0;
b;
c = (3 * K_p * P_0) / (p_0 * R_0);

% Initial Conditions
ic = [0,ddot_0]; % d(0) = 0, d'(0) = ?

% ODE
% R_0 d''(t) + b d'(t) + 3K_p P_0 / p_0 R_0 d(t) = 0 
% a d''(t) + b d'(t) + c d(t) = 0 

% let z(1) = d
% let z(2) = d'

% ∴ dz(1) = d'  = z(2)
% ∴ dz(2) = d'' = (-b d' -c d) / a = (-b z(2) -c z(1)) / a

dz = @(t,z)[ z(2); (-b * z(2) - c*z(1)) / a];

% Evaluate
[t,z]=ode45(dz, tspan, ic);
d = z(:,1);

end


