clear;

SF = 48000; % sample rate
F = 2; % frequency
T = 1; % time

t_in = 0:SF * T;

old_atan2 = 0;
wrap = 0;
inc = 1.0;

for k = 1 + t_in
    left(k) = sin((k * inc) * 2 * pi * F / SF);
    right(k) = cos((k * inc) * 2 * pi * F / SF);

    inc *= 1.00005;

    new_atan2 = atan2(left(k), right(k));
    wrapped(k) = new_atan2;

    if old_atan2 - new_atan2 > pi
        wrap += 1
    end

    old_atan2 = new_atan2;
    unwrapped(k) = wrap * 2 * pi + new_atan2;
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
