clear;

SF = 48000; % sample rate

T = 1; % time

t_in = 0:SF * T;

dir = true;

old_angle = 0;
wrap = 0;
freq = 5.0;
delta = freq / 48000;

for k = 1 + t_in
    s(k) = sin((k * freq) * 2 * pi / SF);
    c(k) = cos((k * freq) * 2 * pi / SF);

    freq -= delta;

    if dir
        left(k) = s(k);
        right(k) = c(k);
    else
        left(k) = c(k);
        right(k) = s(k);
    end

    angle = atan2(left(k), right(k));

    pos(k) = angle;

    if angle - old_angle > pi
        wrap -= 1
    elseif old_angle - angle > pi
        wrap += 1
    end

    old_angle = angle;
    p(k) = SF * (wrap + angle / (2 * pi));
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
plot(p)
plot(pos * 48000 / (2 * pi))
grid on

figure(3)
clf
hold on
plot(pos)
grid on
