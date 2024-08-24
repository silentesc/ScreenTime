const calculateDisplayValue = (value) => {
    value = value / 1000;
    if (value < 60) {
        return `${Math.floor(value)}s`;
    }
    else if (value < 3600) {
        return `${Math.floor(value / 60)}m ${Math.floor(value % 60)}s`;
    }
    else {
        return `${Math.floor(value / 3600)}h ${Math.floor((value % 3600) / 60)}m`;
    }
};

export { calculateDisplayValue };
