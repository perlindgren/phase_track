clear;

SF = 48000; % sample rate

T = 1; % time

t_in = 0:SF * T - 1;

old_atan2 = 0;
wrap = 0;
freq = 5.0;
delta = freq / SF;

for k = t_in
    left(k + 1) = sin(k * freq * 2 * pi / SF);
    right(k + 1) = cos(k * freq * 2 * pi / SF);

    freq -= delta;

    new_atan2 = atan2(left(k + 1), right(k + 1));

    wrapped(k + 1) = new_atan2;

    if new_atan2 - old_atan2 > pi
        wrap -= 1
    elseif old_atan2 - new_atan2 > pi
        wrap += 1
    end

    old_atan2 = new_atan2;
    unwrapped(k + 1) = wrap * 2 * pi + new_atan2;
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
plot(unwrapped)
plot(wrapped)
grid on

figure(3)
clf
hold on
plot(wrapped)
grid on

csvwrite('phase_dec.csv', unwrapped);
