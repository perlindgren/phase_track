clear;

SF = 48000; % sample rate
F = 2; % frequency
T = 1; % time

rand("seed", pi);

t_in = 0:SF * T;

old_atan2 = 0;
wrap = 0;

for k = 1 + t_in
    left(k) = sin(k * 2 * pi * F / SF) + (rand() - 0.5) * 0.5;
    right(k) = cos(k * 2 * pi * F / SF) + (rand() - 0.5) * 0.5;

    new_atan2 = atan2(left(k), right(k));
    wrapped(k) = new_atan2;

    if new_atan2 - old_atan2 > pi
        wrap -= 1
    elseif old_atan2 - new_atan2 > pi
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
