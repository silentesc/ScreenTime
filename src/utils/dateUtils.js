const getCurrentDate = () => {
    const date = new Date();
    return getDateStrFromDate(date);
};

const getDateWithOffset = (dateStr, offset) => {
    const [day, month, year] = dateStr.split('.');
    const date = new Date(`${year}-${month}-${day}`);
    date.setDate(date.getDate() + offset);
    return getDateStrFromDate(date);
}

const getDateStrFromDate = (date) => {
    const day = String(date.getDate()).padStart(2, '0');
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const year = date.getFullYear();
    return `${day}.${month}.${year}`;
};

const isDateStrValid = (dateStr) => {
    let [day, month, year] = dateStr.split('.');
    const date = new Date(`${year}-${month}-${day}`);

    [day, month, year] = getCurrentDate().split('.');
    const currentDate = new Date(`${year}-${month}-${day}`);

    return date <= currentDate;
};

export { getCurrentDate, getDateWithOffset, getDateStrFromDate, isDateStrValid };
