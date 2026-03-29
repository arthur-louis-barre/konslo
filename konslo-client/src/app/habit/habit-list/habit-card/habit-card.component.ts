import {Component, EventEmitter, inject, input, Input, output, Output} from '@angular/core';
import {HabitService} from "../../habit.service";
import {HabitWithCheck} from "../../habit.model";

@Component({
    selector: 'app-habit-card',
    templateUrl: './habit-card.component.html',
    styleUrl: './habit-card.component.css',
})
export class HabitCardComponent {
    private habitsService = inject(HabitService);
    protected isLoading = false;

    habit = input.required<HabitWithCheck>();
    isDropdownOpen = input<boolean>(false);

    habitUpdated = output<void>();
    habitDeleted = output<number>();
    dropdownToggled = output<number>();

    checkHabit() {
        if (this.isLoading) return;
        this.isLoading = true;

        const habit = this.habit();

        const obs = habit.checks.length === 0 ?
            this.habitsService
                .createCheck(habit.id, {value: 1, checked_at: new Date().toISOString()}) :
            this.habitsService
                .updateCheck(habit.id, habit.checks[0].id, {value: habit.checks[0].value + 1});

        obs.subscribe(() => {
            this.habitUpdated.emit();
            this.isLoading = false;
        });
    }

    resetCheckValue() {
        if (this.isLoading) return;
        this.isLoading = true;

        if (this.habit.checks.length === 0) {
            this.isLoading = false;
        } else {
            this.habitsService
                .updateCheck(this.habit.id, this.habit.checks[0].id, {value: 0})
                .subscribe(() => {
                    this.habitUpdated.emit();
                    this.isLoading = false;
                });
        }
    }

    deleteHabit() {
        this.habitDeleted.emit(this.habit.id);
    }



    get progressValue(): number {
        if (this.habit.checks.length === 0) return 0;
        else return this.habit.checks.reduce((sum, check) => sum + check.value, 0);
    }

    get progressPercent() {
        const progressValue = this.progressValue;
        const goalValue = this.habit.goal_value;

        return Math.round((progressValue / goalValue * 100))
    }

    get isCompleted() {
        return this.progressPercent >= 100;
    }
}
