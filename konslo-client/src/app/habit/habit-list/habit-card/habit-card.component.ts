import {Component, inject, input, output} from '@angular/core';
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

    addCheck() {
        if (this.isLoading) return;
        this.isLoading = true;

        const habit = this.habit();

        this.habitsService
            .addCheck(habit.id, 1)
            .subscribe(() => {
                this.habitUpdated.emit();
                this.isLoading = false;
            })
    }

    resetCheck() {
        const habit = this.habit();

        this.habitsService
            .resetCheck(habit.id)
            .subscribe(() => {
                this.habitUpdated.emit();
            });
    }

    deleteHabit() {
        this.habitDeleted.emit(this.habit().id);
    }



    get progressValue(): number {
        if (this.habit().checks.length === 0) return 0;
        else return this.habit().checks.reduce((sum, check) => sum + check.value, 0);
    }

    get progressPercent() {
        const progressValue = this.progressValue;
        const goalValue = this.habit().goal_value;

        return Math.round((progressValue / goalValue * 100))
    }

    get isCompleted() {
        return this.progressPercent >= 100;
    }
}
