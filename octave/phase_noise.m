clear;

SF = 48000; % sample rate
F = 2; % frequency
T = 1; % time

rand("seed", pi);

t_in = 0:SF * T;

dir = true;

old_atan2 = 0;
wrap = 0;

for k = 1 + t_in
    s(k) = sin(k * 2 * pi * F / SF) + (rand() - 0.5) * 0.50;
    c(k) = cos(k * 2 * pi * F / SF) + (rand() - 0.5) * 0.50;

    if dir
        left(k) = s(k);
        right(k) = c(k);
    else
        left(k) = c(k);
        right(k) = s(k);
    end

    wrapped(k) = SF * atan2(left(k), right(k)) / (2 * pi);

    new_atan2 = atan2(left(k), right(k));

    if new_atan2 - old_atan2 > 1.0 * pi
        wrap -= SF
    elseif new_atan2 - old_atan2 < -1.0 * pi
        wrap += SF
    end

    old_atan2 = new_atan2;
    unwrapped(k) = wrap + SF * new_atan2 / (2 * pi);
end

figure(1)
clf
hold on
plot(left)
plot(right)
grid on

figure(2)
clf
hold on
plot(wrapped)
plot(unwrapped)
grid on
