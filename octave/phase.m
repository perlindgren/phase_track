clear;

SF = 48000; % sample rate
F = 2; % frequency
T = 1; % time

t_in = 0:SF * T;

left = sin(t_in * 2 * pi * F / SF);
right = cos(t_in * 2 * pi * F / SF);

old_atan2 = 0;
wrap = 0;

for k = 1 + t_in

    atan2 = atan2(left(k), right(k));

    pos(k) = atan2;

    if old_atan2 - atan2 > pi
        wrap += 1
    end

    old_atan2 = atan2;
    p(k) = SF * (wrap + atan2 / (2 * pi));
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
plot(pos * 48000 / (2 * pi))
plot(p)
grid on

figure(3)
clf
hold on
plot(pos)
grid on
